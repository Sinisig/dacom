# Dacom (Date Conpiler)
A tool for searching for dates in files and performing a statistical analysis of the data.

### Quick Start
 Run the executable from the command line with the --input-file flag to specify the file or directory.
 ```
 dacom --input-file <FILE>
 ```
 Results will be printed to the command line.

### Build Requirements
 - [Cargo]()

### Building
 - Clone the repository locally and enter it from the command line
 - Type the following command to build
 ```
 cargo build --release
 ```
 - The output binary will be target/release/dacom

### About
This tool was made for the purpose of organizing and analyzing the code-comment dates in the leaked Super Mario 64 source code repository.  I wanted to find which files were created and edited when, then organize them accordingly.  I then created this tool to sort all the files from oldest to newest and create statistical information and print it out.  This tool can theoretically be used with any file or folder, but the dates must be in "Month Day Year" format, otherwise the tool will miss them.

