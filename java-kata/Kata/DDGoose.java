package Kata;

public class DDGoose {
    public static String duckDuckGoose(Player[] p, int g) {
        return p[(g-1) % p.length].name;
    }
}
class Player {
    String name;
}
