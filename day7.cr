steps = File.read_lines("day7.input").map do |line|
/Step ([^ ]+).*step ([^ ]+) can begin./.match(line).try do |m|
 {m[1][0], m[2][0]}
end
end.map &.not_nil!

parents = Hash(Char, Set(Char)).new
children = Hash(Char, Set(Char)).new

steps.each do |(p, c)|
	set = parents.fetch(c, Set(Char).new)
	set.add p
	parents[c] = set

	set = children.fetch(p, Set(Char).new)
	set.add c
	children[p] = set
end

chars = Array(Char).new
available = children.select { |key| !parents[key]? }.keys.sort

while available.size > 0
	p = available.shift
	c = (children[p]? || Set(Char).new).to_a.sort

	if chars.includes? p
		next
	end

	chars.push p
	available = available.to_set
	c.each do |v|
		add = true
		if parents[v]?
			add = parents[v].select{ |ch| !chars.includes? ch }.size == 0
		end
		available.add v if add
	end

	available = available.to_a.sort
end

def char_sec(c : Char) : Int32
	return 60 + c.ord - 64
end

puts "Directions #{chars.join("")}"

available = children.select { |key| !parents[key]? }.keys
max_workers = 5
workers = Hash(Char, Int32).new
sec = 0
done = Set(Char).new

while true
	workers.each do |c, s|
		if sec == s
			workers.delete(c)
			done.add c

			children[c]?.try do |children|
				children.map { |c| {c, parents[c] } }.each do |c|
					if c[1].select { |p| !done.includes? p }.size == 0
						available.push c[0]
					end
				end
			end
		end
	end

	available.each do |a|
		if !workers.has_key?(a) && workers.size < max_workers
			workers[a] = char_sec(a) + sec
		end
	end

	workers.keys.each do |k|
		available.delete k
	end

	break if available.size == 0 && workers.size == 0
	sec+=1
end

puts "Seconds: #{sec}"
