from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.so")
res = lib.process(10, 50_000_000)

print(f"Complete! Result={res}")