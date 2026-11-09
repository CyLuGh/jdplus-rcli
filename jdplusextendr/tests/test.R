client <- JDClient$new()
df <- data.frame( date = c("2024-01-01","2024-02-01","2024-03-01"), value = c(10,20,13.5))
res <- client$build_ts_data(df, AggregationType$Sum)$get_data_frame()