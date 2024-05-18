# read bytes file and print it as ascii
import sys
import os 

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 lol.py <filename>")
        sys.exit(1)
    filename = sys.argv[1]
    if not os.path.exists(filename):
        print(f"Error: File {filename} not found")
        sys.exit(1)
    with open(filename, "rb") as f:
        data = f.read()
    print(data.decode("ascii"))
