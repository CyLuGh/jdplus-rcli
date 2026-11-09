library(readxl)

raw <- read_excel("/Users/jed/Documents/GitHub/jdplus-rcli/jdplusextendr/tests/data.xlsx",
                  col_names = FALSE)

# Convert first column to vector
x <- raw[[1]]
# Data starts after the series names
d <- x[-(1:3)]

# Same for second column
y <- raw[[2]]
v <- y[-(1:3)]

df <- data.frame( date = d , value = v)

client <- JDClient$new()
tramo <- client$tramo_full_process_simple_def(df)