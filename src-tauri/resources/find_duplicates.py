#!/usr/bin/env python3

from imagededup.methods import PHash
import sys
import json

"""
This script takes a directory of images as an argument and prints a JSON object
containing a dictionary of duplicate images for the given directory. The keys of
the dictionary are the filenames of the images and the values are a list of
filenames of the duplicate images. The script uses the PHash algorithm to find
duplicates.

Usage: ./duplicates.py <directory>
"""

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <directory>")
    sys.exit(1)

phasher = PHash(False)

# Generate encodings for all images in an image directory
directory = sys.argv[1]
encodings = phasher.encode_images(image_dir=directory)

# Find duplicates using the generated encodings
duplicates = phasher.find_duplicates(encoding_map=encodings)

pruned_duplicates = {}

# Prune circular duplicates
# Example:
# {
#     "file1.jpg": ["file2.jpg", "file3.jpg"],
#     "file2.jpg": ["file1.jpg", "file3.jpg"],
#     "file3.jpg": ["file1.jpg", "file2.jpg"]
# }
# Pruned:
# {
#     "file1.jpg": ["file2.jpg", "file3.jpg"]
# }
for file, duplicates_list in duplicates.items():
    # Create a set of all filenames in the pruned_duplicates dictionary
    # and all filenames in the values of the pruned_duplicates dictionary
    all_names = set(pruned_duplicates.keys()).union(set(value for values in pruned_duplicates.values() for value in values))
    
    # Filter the duplicates_list to exclude filenames that are already in the pruned_duplicates dictionary
    filtered_values = [value for value in duplicates_list if value not in all_names]

    # If the filtered_values list is not empty, add it to the pruned_duplicates dictionary
    if len(filtered_values) > 0:
        pruned_duplicates[file] = filtered_values

# Convert the dictionary into a list of lists, where the key of every element is the first element
pruned_duplicates = [[key] + value for key, value in pruned_duplicates.items()]

# For every filename, prepend the directory to the filename
pruned_duplicates = [[directory + "/" + filename for filename in filenames] for filenames in pruned_duplicates]

print(json.dumps(pruned_duplicates))


