def test(file)
  `cat #{file} | sort                > tmp/test.sort`

  `cat #{file} | ./target/debug/rort > tmp/test.rort`
  `./target/debug/rort #{file} > tmp/test.rort.2`
  results = %x(diff tmp/test.rort tmp/test.sort).split("\n")
  results2 = %x(diff tmp/test.rort.2 tmp/test.sort).split("\n")
  p results + results2
end

test("Cargo.toml")
test("Cargo.lock")
test(".gitignore")
test("src/main.rs")