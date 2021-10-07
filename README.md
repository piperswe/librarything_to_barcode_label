how to use

 1. export your library from https://www.librarything.com/export.php?export_type=tsv
 2. run this command
   
    ```sh
    cargo run --release -- -o labels.html --start n xxx.tsv
    ```

    where n is the starting barcode and xxx.tsv is the path to your exported library