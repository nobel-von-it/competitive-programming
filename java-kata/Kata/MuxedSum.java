package Kata;

import java.util.List;

public class MuxedSum {
    public static int sum(List<?> mixed) {
        return mixed.stream().mapToInt(e -> {
            if (e instanceof String)
                return Integer.parseInt((String) e);
            else return (int) e;
        }).sum();
    }
    public static int sum2(List<?> mixed) {
        return mixed.stream().mapToInt(e -> Integer.parseInt(e.toString())).sum();
    }
}
