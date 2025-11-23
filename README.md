# project-inoculum

## Set Up

1. Create `.env` file with the following variables defined

    ```plaintext
    SURREAL_DB_NAME=
    SURREAL_DB_NAMESPACE=
    SURREAL_DB_PASSWORD=
    SURREAL_DB_URL=
    SURREAL_DB_USERNAME=
    SURREAL_ROOT_PASS=
    SURREAL_ROOT_USER=
    ```

2. Start Database

    ```bash
    docker compose up -d
    ```

3. Create a Namespace with the name defined in the `SURREAL_DB_NAMESPACE` variable from the `.env` file
4. Create a Database with the name defined in the `SURREAL_DB_NAME` variable from the `.env` file
5. Create a system user with `Editor` permission with the `SURREAL_DB_USERNAME` and `SURREAL_DB_PASSWORD` credentials from the `.env` file
6. Run the query defined in the [database_setup.surql](database_setup.surql) file

## Commands

### Log Feed

Command: `log-feed`

Description: Creates an entry of a leaven starter feeding

|    Parameter   |                 Description               |         Flag        | Required | Data Type |
| :------------: | :---------------------------------------: | :-----------------: | :------: | :-------: |
|  flour_amount  |       The amount (in grams) of flour      |  --flour-amount/-f  |   True   |   float   |
| starter_amount |    The amount (in grams) of old starter   | --starter-amount/-s |   True   |   float   |
|  water_amount  |      The amount (in grams) of water       |    --water-amount   |   True   |   float   |
|   water_temp   | The temperature (in Celsius) of the water |     --water-temp    |   True   |   float   |

### Feeding History

Command: `feeding-history`

Description: Provides a timeline of leaven starter feedings

|    Parameter    |                           Description                        |           Flag        | Required | Data Type |
| :-------------: | :----------------------------------------------------------: | :-------------------: | :------: | :-------: |
| maximum_results | The number of starter feedings to return in descending order | --maximum-results/-m  |  False   |    int    |

### Start Dough

Command: `start-dough`

Description: Creates a dough tracking object

| Parameter |                   Description                  |   Flag   | Required | Data Type |
| :-------: | :--------------------------------------------: | :------: | :------: | :-------: |
|    fat    |    The amount (in grams) of fat in the dough   |   --fat  |   False  |   float   |
|   flour   |   The amount (in grams) of flour in the dough  |  --flour |   False  |   float   |
|   leaven  |  The amount (in grams) of leaven in the dough  | --leaven |   False  |   float   |
|    name   |             The dough instance name            |  --name  |   True   |   string  |
|   recipe  |        The recipe the dough is made from       | --recipe |   False  |   string  |
|    salt   |   The amount (in grams) of salt in the dough   |  --salt  |   False  |   float   |
|   scale   | The amount the recipe was scaled for the dough |  --scale |   False  |   float   |
|   sugar   |   The amount (in grams) of sugar in the dough  |  --sugar |   False  |   float   |
|   water   |   The amount (in grams) of water in the dough  |  --water |   False  |   float   |