import sys
import zlib


text=b"""1111111111111111111111111111111111111111111111111112222222222222122313331111111111111111111111111111111111317621356138731827879888888888888888888888812376781263781823789888888888"""

# Checking size of text
text_size=sys.getsizeof(text)
print("\nsize of original text",text_size)

# Compressing text
compressed = zlib.compress(text)

# Checking size of text after compression
csize=sys.getsizeof(compressed)
print("\nsize of compressed text",csize)

# Decompressing text
decompressed=zlib.decompress(compressed)

#Checking size of text after decompression
dsize=sys.getsizeof(decompressed)
print("\nsize of decompressed text",dsize)

print("\nDifference of size= ", text_size-csize)