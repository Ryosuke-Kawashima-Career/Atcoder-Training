import os
import sys

def main(args: str = "0"):
    directory = "../ABC" + args
    if not os.path.exists(directory):
        os.makedirs(directory)
    
    for i in range(7):
        alphabet = chr(ord('A') + i)
        file_path = directory + "/" + "ABC" + args + alphabet + ".rs"
        if os.path.exists(file_path):
            continue
        with open(file_path, 'w') as f:
            f.write("""use proconio::input;

fn main() {
    input!{}
}
""")
    

if __name__ == '__main__':
    if len(sys.argv) > 1:
        main(sys.argv[1])
    else:
        main()
    print("Done!")
