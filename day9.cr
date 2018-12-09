m = /^(\d+) .* (\d+) points$/.match(File.read_lines("day9.input")[0]).not_nil!

player_size, last_marble = m[1].not_nil!.to_i64, m[2].not_nil!.to_i64

def top_score(player_size : Int64, last_marble : Int64)
    circle = Deque.new 1, 0

    scores = Array(Int64).new player_size, 0
    player = 0
    1.upto (last_marble) do |v|
        if v % 23 == 0
            circle.rotate! 7
            removed = circle.shift
            circle.rotate! -1
            scores[player-1] += v + removed
        else
            circle.rotate! -1
            circle.unshift v
        end
        
        player = player == player_size ? 1 : player + 1
    end

    return scores.max
end

#puts "Top score: #{top_score(10, 1618)}"
puts "Top score: #{top_score(player_size, last_marble)}"

puts "Top score (*100): #{top_score(player_size, last_marble*100)}"
