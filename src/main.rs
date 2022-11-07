use std::env;
use std::fs;

use eldarin;

fn main() {
    // Eldarin is a very simple program at its core.
    // It consumes a binary in ELF and produce some information about it.
    // It is assumed to be running on a Linux machine
    //
    // Example(s) of current correct usage:
    // eldarin path/to/filename
    //
    // Example(s) of current incorrect usage:
    // eldarin
    // eldarin filename1 filename2
    // eldarin -f filename

    // Step 1: Get at the content of the specified file
		// Turn commandline arguments into proper arguments
    let args : Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Received too many arguments");
    } else if args.len() == 1 {
        panic!("Received too few arguments");
    }

    let filename = &args[1];
    // println!("trying to open: {}", filename);

    // TODO: Resolve filename or path based on $PATH if filename does not exist in current dir
    // TODO: Find out behavior when the file is a symbolic link
    // Load file content into appropriate array
    let bytes = fs::read(&filename)
        .unwrap_or_else(|_| panic!("Could not reads the content of {}", filename));
    println!("{:?}", bytes);

    // Step 2: Attempt to get at the content of the file
    // Make sure that the file contain enough bits to contain at least an ELF header
		if bytes.len() < 52 {
				panic!("File cannot be a valid ELF file.");
		}

		// Read in the ELF header to a struct
		let elf_header: eldarin::Elf32_Ehdr = unsafe {
				// TODO: Find out if there is a way to make this safer or more understandable
				std::mem::transmute::<[u8; 52], eldarin::Elf32_Ehdr>(bytes[0..52].try_into().expect("incorrect length"))
		};

		for x in 0 .. eldarin::EI_NIDENT {
				println!("{:03}: {}", elf_header.e_ident[x], elf_header.e_ident[x] as char);
		}

		// Convert the Elf header into a proper object that can be investigated more easily

		// Step 3: Perform some analysis

		// Step 4: Getting result back to the user
    // println!("I worked on {filename}");
}
