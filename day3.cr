class Rect
	property left, top, width, height

	def initialize(@id : Int32, @left : Int32, @top : Int32, @width : Int32, @height : Int32)
	end

	def place(store : Array(Int8), width : Int)
		(width * @top).step(to: width * (@height + @top - 1), by: width) do |y|
			(y + @left).upto(y + @left + @width - 1) do |x|
				store[x]+=1
			end
		end
	end
end

list = Array(Rect).new
File.each_line("day3.input", chomp: true) do |line|
	line = line.strip
	parts = line.split(' ')

	coords = parts[2][0..-2].split(",").map{ |v| v.to_i32 }
	dimens = parts[3].split("x").map{ |v| v.to_i32 }
	list.push Rect.new parts[0][1..-1].to_i32, coords[0], coords[1], dimens[0], dimens[1]
end

width = list.map { |r| r.left + r.width }.max + 1
height = list.map { |r| r.top + r.height }.max + 1

fabric = Array(Int8).new width * height, 0
list.each { |r| r.place fabric, width }

puts "Overlap count: #{fabric.count { |v| v > 1 }}"
