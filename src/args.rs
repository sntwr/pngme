use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a secret message into a PNG file
    /// 
    /// The secret message is encoded as a non-critical chunk
    /// inside the PNG file. A single invocation can add a single
    /// secret-message containing chunks. Multiple invocations
    /// can be used to add multiple chunks.
    Encode(EncodeArgs),
    /// Decode the secret message from a PNG file.
    /// 
    /// The message is only extracted and displayed. The input file
    /// is not modified.
    Decode(DecodeArgs),
    /// Remove the embedded secret message(s) from a PNG file
    /// 
    /// A single invocation remove one embedded message chunk.
    /// If there are multiple embedded messages, multiple
    /// invocations need to be used even if they have the same
    /// chunk-type.
    Remove(RemoveArgs),
    /// Dump all chunks inside the PNG file
    /// 
    /// This is useful for debugging. Currently, data is also
    /// dumped as HEX array. The output is NOT easily parseable
    /// programmatically. This might be changed in future!
    Print(PrintArgs),
}
#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the input PNG file
    pub input_file_path: String,
    /// Four byte valid ASCII string for chunk type
    pub chunk_type_str: String,
    /// A UTF-8 message string
    pub message: String,
    /// Path to the output PNG file. If not specified, input file is used
    pub output_file_path: Option<String>,
}
#[derive(Args, Debug)]

pub struct DecodeArgs {
    /// Path to the input PNG file
    pub input_file_path: String,
    /// Four byte valid ASCII string for chunk type
    pub chunk_type_str: String,
}
#[derive(Args, Debug)]

pub struct RemoveArgs {
    /// Path to the input PNG file
    pub input_file_path: String,
    /// Four byte valid ASCII string for chunk type
    pub chunk_type_str: String,
}
#[derive(Args, Debug)]

pub struct PrintArgs {
    /// Path to the input PNG file
    pub input_file_path: String,
}