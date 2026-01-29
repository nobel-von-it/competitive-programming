package Kata;

public class ExesOhs {
    public static boolean getXO(String s) {
        int countX = 0, countO = 0;
        for (String str: s.toLowerCase().split(""))
            switch (str) {
            case "x" -> countX++;
            case "o" -> countO++;
            }
        return countX == countO;
    }
    //or
    public static boolean getXO2(String s) {
        return s.toLowerCase().replace("o", "").length() == s.toLowerCase().replace("x", "").length();
    }
}
