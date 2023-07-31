pragma experimental ABIEncoderV2;

contract Storage {
struct animal {
  string breed;
  uint256 id;
}
    uint256 number;

    /**
     * @dev Store value in variable
     * @param num value to store
     */
    function store(uint256 num) public {
        number = num;
    }

    /**
     * @dev Return value 
     * @return value of 'number'
     */
    function retrieve_123123123123132() public view returns (uint256){
        return number;
    }
    function getName(animal memory myAnimal) public {}
    function Array(address[] memory kek) public {}

    function internalFunction() private {}
}
