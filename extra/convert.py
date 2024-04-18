#!/bin/python3

import os
import sys
import subprocess

def main():
    """
    Main function to convert HEIC to JPG
    Creates a new directory to store the converted files

    "INPUT" directory is the directory where the HEIC files are located
    "OUTPUT" directory is the directory where the JPG files are located
    """
    print("Converting HEIC to JPG")

    input_dir = "./resource/alpha"

    # Create a new directory to store the converted files
    output_dir = "./resource/output"
    os.makedirs(output_dir, exist_ok=True)

    # Convert HEIC to JPG
    for filename in os.listdir(input_dir):
        if filename.endswith(".svg"):
            print(f"Converting {filename} to jpg")
            subprocess.run(["ffmpeg", "-i", f"{input_dir}/{filename}", f"{output_dir}/{filename}.png"])

    print("Conversion complete")

if __name__ == "__main__":
    main()
