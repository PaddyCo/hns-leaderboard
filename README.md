# HnS Leaderboard tool

Generates a cool leaderboard for hack & slash, for use with a BBS

## Example usage

```bash
hns-leaderboard \
    --start="2024-03-01T00:00:00+01:00" \
    --end="2024-04-01T00:00:00+01:00" \
    --greeting-path ./example/greeting.txt \
    --logo-path ./example/logo.txt \
    --start-data ./example/start.json \
    --data ./example/data.json \
    --output-path ./output.txt
```


## Example output


```
    .  _____   Master_of         _____ _         _.  .    . .  .  ..  .    
      |  |  |___ ___| |_   ___  |   __| |___ ___| |_.  . ..        . .     
   +- |     | .'|  _| '_| |   | |__   | | .'|_ -|   |-------------------+  
  .|  |__|__|__,|___|_,_| |_|_| |_____|_|__,|___|_|_| 29% |||---------  |  
  .|                                   Turnering 2024                   |..
  .|                                                                    |..
 ..|  Nu är vi igång igen! Den 1-31 mars 2024 går den sjunde            |  
   |  Hack & Slash-mästerskapet av stapeln, i vanlig ordning är det här |  
  .|  på This Old Cabin BBS som det händer.                             |. 
   |                                                                    |  
 ..| [Topplistan - Dag 9 av 31]             Uppdaterad 2024-03-10 22:47 |. 
  .+--------------------------------------------------------------------+  
  .| SPELARE                      | LEVEL                               |..
   +--------------------------------------------------------------------+. 
 . | 1) Hravnkel                  |  103 |||||||||||||||||||||||||||||| |. 
   | 2) Gedwyn                    |  102 |||||||||||||||||||||||||||||  |  
  .| 3) Lill-Jaun                 |   47 |||||||||||||                  |. 
   | 4) Baronegil                 |   36 ||||||||||                     |. 
   | 5) Beastlord Stintoman       |   14 ||||                           |  
  .| 6) Bosetor                   |   10 ||                             |  
  .| 7) Paddyco                   |    4 |                              |. 
 ..| 8) Tequeida                  |    3                                |. 
  .| 9) Gruj                      |    2                                |. 
   +--------------------------------------------------------------------+  
      ...  . . . ...   . . . . .Besök http://hacknslash.thisoldcabin.net   
     ..     .     .        .                   för en komplett topplista   

```
