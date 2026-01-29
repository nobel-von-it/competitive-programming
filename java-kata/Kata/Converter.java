package Kata;

public class Converter {
    public static int binToDecimal(String s) {
//        s = new StringBuilder(s).reverse().toString();
        int res = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '1')
                res = res * 2 + 1;
            else res *= 2;
        }
        return res;
    }
    public static int binToDecimal2(String s) {
        return Integer.parseInt(s, 2);
    }
}
