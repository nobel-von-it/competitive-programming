package Kata;

public class Binary {
    public static int toBinary(int n) {
        String str = "";
        while (n > 0) {
            str = (n % 2) + str;
            n /= 2;
        }
        return Integer.parseInt(str);
    }
    //or
    public static int toBin(int n) {
        return Integer.parseInt(Integer.toBinaryString(n));
    }
}
