use ethers::prelude::abigen;

abigen!(
  ONFT721NFT,
  r#"[
    function name() public view virtual override returns (string memory)
    function mint(address _tokenOwner, uint _newId) external payable
    function burn(uint256 tokenId) public
    function transferFrom(address from, address to, uint256 tokenId) public
    function safeTransferFrom(address from, address to, uint256 tokenId) public
    function safeTransferFrom(address from, address to, uint256 tokenId, bytes memory data) public
    function approve(address to, uint256 tokenId) public
    function setApprovalForAll(address operator, bool approved) public
    function getApproved(uint256 tokenId) public view returns (address)
    function isApprovedForAll(address owner, address operator) public view returns (bool)
    function ownerOf(uint256 tokenId) public view returns (address)
    function balanceOf(address owner) public view returns (uint256)
    function tokenURI(uint256 tokenId) public view returns (string memory)
    function estimateSendFee(uint16 _dstChainId, bytes calldata _toAddress, uint _tokenId, bool _useZro, bytes calldata _adapterParams ) external view returns (uint nativeFee, uint zroFee)
    function sendFrom(address _from, uint16 _dstChainId, bytes calldata _toAddress, uint _tokenId, address payable _refundAddress, address _zroPaymentAddress, bytes calldata _adapterParams) external payable
  ]"#,
);

abigen!(
  ONFT1155NFT,
  r#"[
    function mint(address _to, uint _tokenId, uint _amount) public
    function burn(address account, uint256 tokenId, uint256 amount) public
    function balanceOf(address account, uint256 id) public view returns (uint256)
    function balanceOfBatch(address[] memory accounts, uint256[] memory ids) public view returns (uint256[] memory)
    function setApprovalForAll(address operator, bool approved) public
    function isApprovedForAll(address account, address operator) public view returns (bool)
    function safeTransferFrom(address from, address to, uint256 id, uint256 amount, bytes memory data) public
    function safeBatchTransferFrom(address from, address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data) public
    function estimateSendFee( uint16 _dstChainId, bytes calldata _toAddress, uint _tokenId, uint _amount, bool _useZro, bytes calldata _adapterParams) external view returns (uint nativeFee, uint zroFee)
    function sendFrom(address _from, uint16 _dstChainId, bytes calldata _toAddress, uint _tokenId, address payable _refundAddress, address _zroPaymentAddress, bytes calldata _adapterParams) external payable
  ]"#,
);

abigen!(
  NFTMatket,
  r#"[
    function listNFT(address _NFTContract, uint256 _tokenId, uint256 _price) public
    function buyNFT(address _NFTContract, uint256 _tokenId) public payable
    function list1155NFT(address _NFTContract, uint256 _tokenId, uint256 _amount, uint256 _price) public
    function buy1155NFT(address _NFTContract, uint256 _tokenId, address seller, uint256 _amount) public payable
  ]"#,
);