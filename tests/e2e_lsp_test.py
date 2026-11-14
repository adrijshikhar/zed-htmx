#!/usr/bin/env python3
import json
import queue
import subprocess
import sys
import threading
import time
from pathlib import Path

class ProcessReader:
    """Threaded reader queue to safely and portably read stdout on all platforms (POSIX + Windows)."""
    def __init__(self, stdout):
        self.stdout = stdout
        self.queue = queue.Queue()
        self.thread = threading.Thread(target=self._reader_thread, daemon=True)
        self.thread.start()

    def _reader_thread(self):
        try:
            while True:
                ch = self.stdout.read(1)
                if not ch:
                    self.queue.put((None, EOFError("stdout closed")))
                    break
                self.queue.put((ch, None))
        except Exception as e:
            self.queue.put((None, e))

    def read_exact(self, count, timeout=5):
        start = time.time()
        buf = bytearray()
        while len(buf) < count:
            remaining = timeout - (time.time() - start)
            if remaining <= 0:
                raise TimeoutError("Timed out reading data from server")
            try:
                ch, err = self.queue.get(timeout=remaining)
            except queue.Empty:
                raise TimeoutError("Timed out waiting for server output")
            if err:
                raise err
            buf.extend(ch)
        return bytes(buf)

    def read_until(self, delimiter: bytes, timeout=5):
        start = time.time()
        buf = bytearray()
        while delimiter not in buf:
            remaining = timeout - (time.time() - start)
            if remaining <= 0:
                raise TimeoutError(f"Timed out waiting for header delimiter: {buf!r}")
            try:
                ch, err = self.queue.get(timeout=remaining)
            except queue.Empty:
                raise TimeoutError("Timed out waiting for server output")
            if err:
                raise err
            buf.extend(ch)
        return bytes(buf)

def send_msg(proc, payload):
    body = json.dumps(payload).encode("utf-8")
    header = f"Content-Length: {len(body)}\r\n\r\n".encode("utf-8")
    proc.stdin.write(header + body)
    proc.stdin.flush()

def read_msg(reader, timeout=5):
    header_bytes = reader.read_until(b"\r\n\r\n", timeout=timeout)
    headers = header_bytes.decode("utf-8")
    content_len = 0
    for line in headers.split("\r\n"):
        if line.lower().startswith("content-length:"):
            content_len = int(line.split(":")[1].strip())

    body = reader.read_exact(content_len, timeout=timeout)
    return json.loads(body.decode("utf-8"))

def main():
    print("=== STARTING E2E LSP PROTOCOL VERIFICATION ===")
    repo_root = Path(__file__).resolve().parent.parent
    server_path = "htmx-analyzer"
    print(f"[1] Spawning server process: {server_path}")
    proc = subprocess.Popen(
        [server_path, "--stdio"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    )
    reader = ProcessReader(proc.stdout)

    try:
        # Step 1: Initialize
        print("[2] Sending 'initialize' request...")
        init_req = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": None,
                "rootUri": repo_root.as_uri(),
                "capabilities": {
                    "textDocument": {
                        "hover": {"contentFormat": ["markdown"]},
                        "completion": {"completionItem": {"snippetSupport": True}},
                        "codeAction": {"codeActionLiteralSupport": {}}
                    }
                }
            }
        }
        send_msg(proc, init_req)
        init_resp = read_msg(reader)
        assert init_resp["id"] == 1, f"Expected id 1, got {init_resp}"
        caps = init_resp["result"]["capabilities"]
        assert "hoverProvider" in caps, "Missing hoverProvider in capabilities"
        assert "completionProvider" in caps, "Missing completionProvider in capabilities"
        assert "codeActionProvider" in caps, "Missing codeActionProvider in capabilities"
        print("    -> Handshake OK: Server capabilities verified.")

        # Step 2: Initialized notification
        send_msg(proc, {"jsonrpc": "2.0", "method": "initialized", "params": {}})

        # Step 3: didOpen test.html
        test_file_path = Path(__file__).resolve().parent / "test.html"
        test_file_uri = test_file_path.as_uri()
        test_content = test_file_path.read_text(encoding="utf-8")

        print("[3] Sending 'textDocument/didOpen' for test.html...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": test_file_uri,
                    "languageId": "html",
                    "version": 1,
                    "text": test_content
                }
            }
        })

        # Expect diagnostics publication
        diag_msg = read_msg(reader)
        assert diag_msg["method"] == "textDocument/publishDiagnostics"
        diags = diag_msg["params"]["diagnostics"]
        print(f"    -> Diagnostics on initial valid test.html: {len(diags)} (expected 0)")
        assert len(diags) == 0, f"Expected clean diagnostics, got: {diags}"

        # Step 4: Hover on hx-get (line 7, character 14)
        print("[4] Testing 'textDocument/hover' on hx-get (line 7, character 14)...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/hover",
            "params": {
                "textDocument": {"uri": test_file_uri},
                "position": {"line": 7, "character": 14}
            }
        })
        hover_resp = read_msg(reader)
        assert hover_resp["id"] == 2
        hover_val = hover_resp["result"]["contents"]["value"]
        assert "hx-get" in hover_val, f"Expected hx-get in hover, got {hover_val}"
        assert "Issues a GET request" in hover_val
        print("    -> Hover OK: Found hx-get documentation & URL.")

        # Step 5: Hover on hx-target (line 7, character 35)
        print("[5] Testing 'textDocument/hover' on hx-target (line 7, character 35)...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/hover",
            "params": {
                "textDocument": {"uri": test_file_uri},
                "position": {"line": 7, "character": 35}
            }
        })
        hover_resp = read_msg(reader)
        assert hover_resp["id"] == 3
        hover_val = hover_resp["result"]["contents"]["value"]
        assert "hx-target" in hover_val
        print("    -> Hover OK: Found hx-target documentation.")

        # Step 6: Hover on outerHTML swap style (line 7, character 65)
        print("[6] Testing 'textDocument/hover' on outerHTML (line 7, character 65)...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "id": 4,
            "method": "textDocument/hover",
            "params": {
                "textDocument": {"uri": test_file_uri},
                "position": {"line": 7, "character": 65}
            }
        })
        hover_resp = read_msg(reader)
        assert hover_resp["id"] == 4
        hover_val = hover_resp["result"]["contents"]["value"]
        assert "outerHTML" in hover_val
        assert "Swap Style" in hover_val
        print("    -> Hover OK: Found outerHTML swap style documentation.")

        # Step 7: Completion at line 7, character 14
        print("[7] Testing 'textDocument/completion'...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "id": 5,
            "method": "textDocument/completion",
            "params": {
                "textDocument": {"uri": test_file_uri},
                "position": {"line": 7, "character": 14}
            }
        })
        comp_resp = read_msg(reader)
        assert comp_resp["id"] == 5
        res = comp_resp["result"]
        items = res if isinstance(res, list) else res["items"]
        labels = [item["label"] for item in items]
        assert "hx-get" in labels, f"Expected hx-get in completion labels: {labels}"
        print(f"    -> Completion OK: Returned {len(items)} items matching 'hx-g' prefix: {labels}.")

        # Step 8: Edit file to introduce deprecated attribute & invalid swap style
        print("[8] Sending 'textDocument/didChange' with deprecated hx-ws and bad hx-swap...")
        buggy_content = test_content.replace(
            'hx-get="/clicked"',
            'hx-ws="connect:/chat" hx-swap="invalidSwapStyle"'
        )
        send_msg(proc, {
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {
                    "uri": test_file_uri,
                    "version": 2
                },
                "contentChanges": [
                    {"text": buggy_content}
                ]
            }
        })

        # Expect updated diagnostics
        diag_msg = read_msg(reader)
        assert diag_msg["method"] == "textDocument/publishDiagnostics"
        diags = diag_msg["params"]["diagnostics"]
        print(f"    -> Diagnostics updated: {len(diags)} reported")
        codes = [d["code"] for d in diags]
        print(f"    -> Diagnostic codes: {codes}")
        assert "HTMX001" in codes, "Expected HTMX001 for deprecated hx-ws"
        assert "HTMX002" in codes, "Expected HTMX002 for invalid swap style"

        # Step 9: Code actions on HTMX001 (deprecated hx-ws)
        ws_diag = next(d for d in diags if d["code"] == "HTMX001")
        print("[9] Testing 'textDocument/codeAction' for HTMX001 quick-fix...")
        send_msg(proc, {
            "jsonrpc": "2.0",
            "id": 6,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": {"uri": test_file_uri},
                "range": ws_diag["range"],
                "context": {
                    "diagnostics": [ws_diag]
                }
            }
        })
        ca_resp = read_msg(reader)
        assert ca_resp["id"] == 6
        actions = ca_resp["result"]
        assert len(actions) > 0, "Expected at least 1 code action"
        ca = actions[0]
        assert "Migrate hx-ws" in ca["title"]
        edits = ca["edit"]["changes"][test_file_uri]
        assert edits[0]["newText"] == 'hx-ext="ws" ws-connect="/chat"'
        print(f"    -> CodeAction OK: '{ca['title']}' -> '{edits[0]['newText']}'")

        # Step 10: Shutdown & Exit
        print("[10] Testing clean shutdown & exit protocol...")
        send_msg(proc, {"jsonrpc": "2.0", "id": 7, "method": "shutdown", "params": None})
        shutdown_resp = read_msg(reader)
        assert shutdown_resp["id"] == 7
        assert shutdown_resp["result"] is None
        send_msg(proc, {"jsonrpc": "2.0", "method": "exit", "params": None})
        proc.wait(timeout=3)
        assert proc.returncode == 0, f"Expected exit code 0, got {proc.returncode}"
        print("    -> Shutdown & Exit OK: Server exited cleanly.")

        print("\nALL E2E PROTOCOL TESTS PASSED WITH 100% SUCCESS!")

    finally:
        if proc.poll() is None:
            proc.kill()

if __name__ == "__main__":
    main()
