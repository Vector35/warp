#!/usr/bin/env python3

import os
import xtarfile
import tarfile
from operator import contains
from ar import Archive


def extract_deb_files_in_dir(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.deb'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'rb') as f:
                        archive = Archive(f)
                        entry_dir = file_path.strip(".deb") + "/"
                        os.makedirs(entry_dir, exist_ok=True)
                        for entry in archive:
                            entry_path = entry_dir + entry.name
                            if os.path.exists(entry_path):
                                print('Skipping existing deb entry', entry_path)
                                continue
                            with open(entry_path, 'wb') as output:
                                content = archive.open(entry, 'rb').read()
                                output.write(content)
                except IOError as e:
                    print(f"Failed to extract {file_path}: {e}")

def extract_tar_files_in_dir(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            file_path = os.path.join(root, file)
            if contains(file_path, '.tar'):
                extract_dir = file_path.split('.tar')[0]
                try:
                    if os.path.exists(extract_dir):
                        print('Skipping existing tar extract', extract_dir)
                        continue
                    os.makedirs(extract_dir, exist_ok=True)
                    with xtarfile.open(file_path, mode='r') as tar:
                        tar.extractall(path=extract_dir)
                except tarfile.TarError as e:
                    print(f"Failed to extract {file_path}: {e}")

# Run the extraction in the current working directory
if __name__ == "__main__":
    cwd = os.getcwd()
    extract_deb_files_in_dir(cwd)
    extract_tar_files_in_dir(cwd)
    
