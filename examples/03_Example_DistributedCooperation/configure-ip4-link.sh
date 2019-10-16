#! /bin/bash

device=$(ip -4 link|grep -e LOWER_UP|grep -v -e UNKNOWN -e DORMANT -e noqueue|cut -d: -f2)

echo device $device

setdevice () {
    xmltmpl=$1
    xmlfile=$(echo $xmltmpl|sed 's/-template//')
    xsltproc --stringparam device $device <(cat <<'EOD'
<xsl:stylesheet version="1.1" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:param name = "device"/>

    <!-- copy all nodes and attributes -->
    <xsl:template match="@*|node()" name = "identity">
        <xsl:copy >
            <xsl:apply-templates select="@*|node()"/>
        </xsl:copy>
    </xsl:template>

    <xsl:template match="/UxAS/Bridge/@NetworkDevice">
        <xsl:attribute name="NetworkDevice">
            <xsl:value-of select = "$device"/>
        </xsl:attribute>
    </xsl:template>

</xsl:stylesheet>
EOD
    ) $xmltmpl >$xmlfile
    echo wrote $xmlfile
}

setdevice cfgDistributedCooperation_1000-template.xml
setdevice cfgDistributedCooperation_2000-template.xml
