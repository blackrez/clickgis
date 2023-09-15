# Clickgis Project

Actually clickhouse have a very basic support of geo function.
The goal of this project is to add openGIS functions.
I choose to use georust because it is simplier to use than geos one.

## Step 1: make it work

First step is to write the functions only `st_aswkb` and `st_asgeojson` are supported for now.
UDF are used to test if functions work.

First build, the clickgis command

```
cargo build
```

Move the binary clickgis to the `user_scripts_path`.

Modify the `user_defined_executable_functions_config` file to defined the GIS functions.

```
<functions>
    <function>
        <type>executable_pool</type>
        <name>st_aswkt</name>
        <return_type>String</return_type>
	<argument>
            <type>String</type>
            <name>value</name>
        </argument>
        <format>TabSeparated</format>
        <command>clickgis --function st_aswkt</command>
    </function>
    <function>
	    <type>executable</type>
	    <name>st_asgeojson</name>
	    <return_type>String</return_type>
	    <argument>
		    <type>String</type>
		    <name>value</name>
	    </argument>
	    <format>TabSeparated</format>
	    <command>clickgis --function st_asgeojson</command>
	</function>
</functions>
```

Launch clickhouse (or clickhouse local)

```
./clickhouse local -- --user_scripts_path=functions --user_defined_executable_functions_config=function.xml
```

Use it 

```
ClickHouse local version 23.9.1.554 (official build).

macbook-air-de-nabil.home :) SELECT addresses, names, st_asgeojson(hex(geometry))
FROM s3('https://overturemaps-us-west-2.s3.amazonaws.com/release/2023-07-26-alpha.0/theme=places/type=*/*', '', '', 'Parquet')
WHERE       tupleElement(bbox, 'minx') > 5.203050
   AND tupleElement(bbox, 'maxx') < 5.322269
   AND tupleElement(bbox, 'miny') > 43.630781
   AND  tupleElement(bbox, 'maxy') < 43.674811 LIMIT 10;
```



## Step 2 : integrate to clickhouse

Clickhouse already rust for `blacke2`.
Will add when more functions will be supported.

https://clickhouse.com/docs/en/development/integrating_rust_libraries

