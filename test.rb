def test(stdin: nil, files: [])
  `#{"cat #{stdin} | " if stdin}sort #{files.join(" ")}> tmp/test.sort`
  `#{"cat #{stdin} | " if stdin}./target/debug/rort #{files.join(" ")}> tmp/test.rort`
  results = %x(diff tmp/test.rort tmp/test.sort).split("\n")
  p results
end

test(stdin: "Cargo.toml")
test(stdin: "Cargo.lock")
test(stdin: ".gitignore")
test(stdin: "src/main.rs")
test(files: ["src/main.rs"])
test(files: ["src/main.rs", "Cargo.toml"])