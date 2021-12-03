import java.util.*;
import java.io.*;
import java.util.stream.*;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new FileReader("input.txt"));
        var splits = br.lines().collect(Collectors.toList());
        long h = 0;
        long d = 0;
        long aim = 0;

        for (int i = 0; i < splits.size(); i++) {
            var curr = splits.get(i).split(" ");
            var direction = curr[0];
            var amount = Integer.valueOf(curr[1]);

            if (direction.equals("forward")) {
                h += amount;
                d += aim * amount;
            } else if (direction.equals("up")) {
                aim -= amount;
            } else if (direction.equals("down")) {
                aim += amount;
            }
        }

        System.out.println(h*d);
    }
}
