package se.hof.adventofcode.eighteen;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class DayEightTest {

    @Test
    public void parseChildlessNodeTest() {
        String input = "0 1 99";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 99;
        assertEquals(expectedSum, n.sum());
    }

    @Test
    public void parseChildlessNodeTest2() {
        String s = "0 3 10 11 12";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(33, n.sum());
    }


    @Test
    public void parseNodeWithChild() {
        String s = "1 1 0 1 99 2";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(101, n.sum());
    }

    @Test
    public void parseNodeWithNChildren() {
        String s = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        DayEight.Node n = DayEight.Node.parse(s);
        assertEquals(2, n.children.size());
        assertEquals(138, n.sum());
    }

}
