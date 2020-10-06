use io::Write;
use std::{
    borrow::Cow, env, fs::File, io, path::Path, path::PathBuf, process::Command, process::Stdio,
};

/// taken from bindgen
///
///
/// Gets the rustfmt path to rustfmt the generated bindings.
fn rustfmt_path() -> PathBuf {
    if let Ok(rustfmt) = env::var("RUSTFMT") {
        return rustfmt.into();
    }
    which::which("rustfmt").expect("path to rustfmt not found")
}

/// Checks if rustfmt_bindings is set and runs rustfmt on the string
fn rustfmt_generated_string<'a>(source: &'a str) -> io::Result<Cow<'a, str>> {
    let rustfmt = rustfmt_path();
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let source = source.to_owned();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output)?;

    let status = child.wait()?;
    let source = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
    );

    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(Cow::Owned(bindings)),
            Some(2) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors.".to_string(),
            )),
            Some(3) => {
                eprintln!("Rustfmt could not format some lines.");
                Ok(Cow::Owned(bindings))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            )),
        },
        _ => Ok(Cow::Owned(source)),
    }
}

pub fn write_formatted(file: &Path, unformatted: &str) {
    File::create(file)
        .expect("failed to create file")
        .write_all(
            rustfmt_generated_string(unformatted)
                .expect("unable to format")
                .as_bytes(),
        )
        .expect("unable to write formatted");
}
