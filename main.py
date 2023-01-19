import time
from collections import Counter
from txtvectors import multiply, frequency_encode, batch_frequency_encode

s = "the quick brown fox jumps over the lazy dog and the fox jumps over the fox dog"

print(multiply(12, 14))

print("--- Frequency Encode ---")
tic = time.perf_counter()
for _ in range(1_000):
    res = frequency_encode(s)
toc = time.perf_counter()
print(res)
print(f"Time taken: {toc - tic:0.9f} seconds")

print("--- Batch Frequency Encode ---")
tic = time.perf_counter()
res = batch_frequency_encode([s] * 1000)
toc = time.perf_counter()
print(res)
print(f"Time taken: {toc - tic:0.9f} seconds")


print("--- Counter ---")
tic = time.perf_counter()
for _ in range(1_000):
    res = dict(Counter(s.split(" ")))
toc = time.perf_counter()
print(res)
print(f"Time taken: {toc - tic:0.9f} seconds")
