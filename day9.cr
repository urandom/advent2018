m = /^(\d+) .* (\d+) points$/.match(File.read_lines("day9.input")[0]).not_nil!

player_size, last_marble = m[1].not_nil!.to_i, m[2].not_nil!.to_i
#player_size, last_marble = 9, 25
circle = Array.new 1, 0

scores = Array.new player_size, 0
current_idx = 0
player = 0
1.upto last_marble do |v|
    if v % 23 == 0
        scores[player-1] += v
        if current_idx >= 7
            scores[player-1] += circle.delete_at current_idx - 7
            current_idx -= 7
        else
            scores[player-1] += circle.delete_at circle.size + current_idx - 7
            current_idx = circle.size + current_idx - 6
            current_idx = 0 if current_idx >= circle.size
        end
    elsif circle[current_idx+2]?
        circle.insert current_idx+2, v
        current_idx = current_idx+2
    elsif circle[current_idx+1]?
        circle.push v
        current_idx = current_idx+2
    else
        circle.insert 1, v
        current_idx = 1
    end
    
    player = player == player_size ? 1 : player + 1
end

puts "Top score: #{scores.max}"
