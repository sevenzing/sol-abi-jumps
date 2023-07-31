import {Ownable} from "./owner.sol";


contract MainContract is Ownable {
    function my_id() public pure returns (uint256){
        return 12312391892;
    }

    function func_with_args(uint256 num) public returns (uint256) {
        return my_id() + num;
    }

    function internalFunction() private {}
}