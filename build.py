import os
import shutil
import sys

compile_commands = dict({
    "linux": "cd rust && cargo build --target x86_64-unknown-linux-gnu --release",
    "windows": "cd rust && cargo build --target x86_64-pc-windows-gnu --release",
    "apple": "cd rust && cargo build --target x86_64-apple-darwin --release" 

})

result_name = dict({
    "linux" : "x86_64-unknown-linux-gnu/release/libsmb_rust.so",
    "windows" : "x86_64-pc-windows-gnu/release/smb_rust.dll"
})

# Used to keep the logging style consistant
def log(text):
    print(f"# {text}")

# Compiles the rust source on linux and copy the .so file to the godot directory
def compile_rust(target):
    log(f"Compiling the rust source for {target}")
    os.system(compile_commands[target])
    
    log("Copying files")
    shutil.move("rust/target/" + result_name[target], "godot/pkg/" + result_name[target])
    log("Done")

def main():
    if sys.argv[1] == "all" :
        compile_rust("linux")
        compile_rust("windows")
        return

    compile_rust(sys.argv[1])

if __name__ == "__main__":
    main()