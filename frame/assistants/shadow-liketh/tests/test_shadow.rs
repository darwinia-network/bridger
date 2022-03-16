mod common;

#[tokio::test]
async fn test_mmr_root() {
    let shadow = common::shadow();
    shadow.mmr_root(12092405);
    /*
    {
      nodeEntities(
        first: 5
        orderBy: id,
        orderDirection: desc
      where: {
        id_in:

        [16777214, 20971517, 23068668, 24117243, 24182778, 24183801, 24184312, 24184567, 24184694, 24184757, 24184788, 24184795, 24184796]

      }
      ) {
        id
        position
        hash
      }
    }
     */
}
