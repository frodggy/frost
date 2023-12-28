local xml = frost.xml

return xml.html {charset="utf-8", lang="en"} {
    xml.head {
        xml.title "Hello World"
    },
    xml.body {
        xml.h1 "Hello World",

        xml.div {id="numbers"} {
            function() --run as a coroutine
                for i = 1, 10 do
                    coroutine.yield(xml.p(i))
                end
            end
        }
    }
}
