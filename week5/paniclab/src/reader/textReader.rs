enum Tone {
    Neutral,
    Happy,
    Sad,
    Angry,
    Surprised,
}

struct TextReader {
    sender: String,
    msg: String,
    receiver: String,
}

impl TextReader {
    /// Creates a new TextReader
    /// file_path - is the path to the text message that will be read
    /// The file should be in the following format:
    /// From: <sender>
    /// <message>
    /// To: <receiver>
    /// Use the use std::fs and std::io to read the file
    /// and get the sender, receiver, and message
    ///
    /// If the file does not exist, panic with the message "File not found"
    /// If the file is not in the correct format, panic with the message "File is not in correct format"
    /// If no message is found, assume the message is ""
    fn new(file_path: String) -> TextReader {
        let file = File::open(file_path).expect("File not found");

        let mut sender = String::new();
        let mut msg = String::new();
        let mut receiver = String::new();

        // Create a buffered reader to read the file line by line
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            let line = line?;

            println!("{}", line);
        }
    }
}
