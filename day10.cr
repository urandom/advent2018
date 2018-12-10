class Point
	getter x, y

	def initialize(@x : Int32, @y : Int32, @vx : Int32, @vy : Int32)
	end

	def move
		@x += @vx
		@y += @vy
	end
end

def vertical(points : Array(Point), height = 5) : Bool
	cols = points.group_by(&.x).values.map(&.map(&.y).sort)

	prev = nil
	h = 1
	cols.each do |col|
		col.each do |y|
			if prev
				if y - prev == 1
					h += 1
					return true if h == height
				elsif y - prev > 0
					h = 1
				end
				prev = y
			else
				prev = y
			end
		end
	end
	return false
end

def chart(points : Array(Point))
	min_x = points.min_by(&.x).x
	min_y = points.min_by(&.y).y
	max_x = points.max_by(&.x).x
	max_y = points.max_by(&.y).y

	rows = points.group_by &.y
	min_y.upto(max_y).each do |y|
		xx = rows[y]? ? rows[y].map(&.x).sort : Array(Int32).new
		min_x.upto(max_x).each do |x|
			print (rows.has_key?(y) && xx.includes? x) ? "#" : '.'
		end
		print "\n"
	end
	print "\n"
end

points = File.read_lines("day10.input").map do |l|
	m = l.split(/([-]?\d+)/).not_nil!

	Point.new m[1].not_nil!.to_i, m[3].not_nil!.to_i, m[5].not_nil!.to_i, m[7].not_nil!.to_i
end

while true
	points.each &.move

	if vertical points, 8
		chart points
		break
	end
end

