package Kata;

public class AllStar {
    public static int strCount(String s, char l) {
        int count = 0;
        for (String t : s.split(""))
            if (t.equals(String.valueOf(l)))
                count++;
        return count;
    }
    // or
    public static int strCountt(String s, char l) {
        return (int) s.chars().filter(x -> x == l).count();
    }
}
