import com.rpn2tex.*;

public class TestTokenFormat {
    public static void main(String[] args) {
        Token t1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token t2 = new Token(TokenType.PLUS, "+", 1, 4);
        Token t3 = new Token(TokenType.EOF, "", 1, 5);
        
        System.out.println(t1.toString());
        System.out.println(t2.toString());
        System.out.println(t3.toString());
    }
}
