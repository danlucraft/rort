def test(file)
  `cat #{file} | ./target/debug/rort > tmp/test.rort`
  `cat #{file} | sort                > tmp/test.sort`
  results = %x[diff tmp/test.rort tmp/test.sort].split("\n")
  p results
end

test("Cargo.toml")
test("Cargo.lock")
test(".gitignore")
test("src/main.rs")