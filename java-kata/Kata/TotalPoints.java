package Kata;

import java.util.Arrays;

public class TotalPoints {
    public static void main(String[] args) {
        System.out.println(points(new String[]
                {"1:0","2:0","3:0","4:0","2:1","3:1","4:1","3:2","4:2","4:3"}));
    }
    public static int points(String[] games) {
        int result =Arrays.stream(games).map(el -> {
            String[] els = el.split(":");
            return Integer.parseInt(els[0]) > Integer.parseInt(els[1]) ?
                    3 : Integer.parseInt(els[0]) == Integer.parseInt(els[1]) ?
                    1 : 0;
        }).reduce(Integer::sum).get();
        return result;
    }
}
