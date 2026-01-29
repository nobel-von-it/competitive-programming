package Kata;

public class SquareDigit {
    public int squareDigits(int n) {
        String[] s = String.valueOf(n).split("");
        StringBuilder res = new StringBuilder();

        for (String string : s) {
            res.append((int) Math.pow(Integer.parseInt(string), 2));
        }
        return Integer.parseInt(res.toString());
    }
    public int sD(int n){
        int res = 0;
        for (char c : String.valueOf(n).toCharArray()) {
            res += (int) Math.pow(Character.digit(c, 10), 2);
        }
        return res;
    }
}
