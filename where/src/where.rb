#! /usr/bin/env ruby

progname = File.basename($0)
usage = "usage: #{progname} <...>"

pattern = ARGV.shift
if not pattern
  $stderr.puts(usage)
  exit(2)
end

def matchdir(dir, pattern)
  entries = (Dir.entries(dir) - ['.', '..']).grep(/#{pattern}/)
  entries.map { |entry| File.join(dir, entry) }
end

ENV.fetch('PATH', []).split(ENV.fetch('IFS', ':')).each do |pathelem|
  next  unless Dir.exist?(pathelem)

  matches = matchdir(pathelem, pattern)
  matches.each do |m|
    next  unless File.exist?(m) && File.executable?(m)
    puts m
  end
end
