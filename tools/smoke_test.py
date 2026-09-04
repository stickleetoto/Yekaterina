#!/usr/bin/env python3
import json, subprocess, sys, threading, queue, time

def send(p, obj):
    p.stdin.write(json.dumps(obj, separators=(",", ":")) + "\n")
    p.stdin.flush()

def recv(q, want_id, timeout=8):
    end=time.time()+timeout
    while time.time()<end:
        try: obj=q.get(timeout=max(0.05,end-time.time()))
        except queue.Empty: continue
        if isinstance(obj,dict) and obj.get("id")==want_id: return obj
    raise RuntimeError(f"timeout waiting for id={want_id}")

def main():
    if len(sys.argv)!=2:
        print("usage: smoke_test.py <path-to-yekaterina.exe>", file=sys.stderr); return 2
    exe=sys.argv[1]
    p=subprocess.Popen([exe],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,bufsize=1)
    q=queue.Queue()
    def reader():
        for line in p.stdout:
            try: q.put(json.loads(line))
            except Exception: pass
    threading.Thread(target=reader,daemon=True).start()
    try:
        send(p,{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"yekaterina-public-smoke","version":"1.0"}}})
        init=recv(q,1)
        if "error" in init: raise RuntimeError(init["error"])
        send(p,{"jsonrpc":"2.0","method":"notifications/initialized","params":{}})
        send(p,{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})
        tools=recv(q,2)
        names=[x.get("name") for x in tools.get("result",{}).get("tools",[])]
        expected=["yk.compute","yk.find","yk.spec"]
        if sorted(names)!=sorted(expected): raise RuntimeError(f"unexpected tools: {names}")
        send(p,{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"yk.compute","arguments":{"op":"math.add","a":[20,22]}}})
        out=recv(q,3)
        if "error" in out: raise RuntimeError(out["error"])
        txt=" ".join(x.get("text","") for x in out.get("result",{}).get("content",[]) if isinstance(x,dict))
        if "42" not in txt: raise RuntimeError(f"math.add smoke did not contain 42: {txt!r}")
        print("MCP tools:", ", ".join(names))
        print("math.add smoke: 42")
        print("PASS")
        return 0
    finally:
        p.kill()

if __name__=="__main__": raise SystemExit(main())
