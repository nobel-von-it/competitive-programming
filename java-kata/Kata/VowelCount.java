package Kata;

public class VowelCount {
    public static int getCount(String s) {
        if (s.isEmpty())
            return 0;
        int count = 0;
        for (String str:
             s.split("")) {
            if ("aeiou".contains(str))
                count++;
        }
        return count;
    }
    // or idn
    public static int getCountt(String s) {
        return s.replaceAll("(?i)[^aeiou]", "").length();
    }
}
