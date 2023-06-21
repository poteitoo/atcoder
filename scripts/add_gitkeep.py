import os

def find_dir_recursive(base_path, dir_name):
    target_roots = []
    for root, dirs, files in os.walk(base_path):
        if dir_name in dirs:
            target_roots.append(os.path.join(root,dir_name))
    return target_roots


def create_gitkeep_file(folder_path):
    gitkeep_file = os.path.join(folder_path, ".gitkeep")
    with open(gitkeep_file, "w"):
        pass

folder_path = ".."
in_dirs = find_dir_recursive(folder_path,'in')
out_dirs = find_dir_recursive(folder_path,'out')
for target_dir in zip(in_dirs, out_dirs):
    create_gitkeep_file(target_dir[0])
    create_gitkeep_file(target_dir[1])
