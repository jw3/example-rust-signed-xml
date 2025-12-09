example rust signed xml
===

Signing and verifying XML documents with the `quick-xml` and `rsa` crates.

## run

`$ example-sign-verify test/data/sample-input.xml`
****
## sample input

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Document>
  <Header id="h1">
    <Title>Sample Document</Title>
    <Author>John Doe</Author>
  </Header>
  <Body>
    <Section name="intro">
      <Paragraph>This is the first paragraph.</Paragraph>
      <Paragraph>This is the second paragraph.</Paragraph>
    </Section>
    <Section name="conclusion">
      <Paragraph>This is the conclusion.</Paragraph>
    </Section>
  </Body>
</Document>
```

## sample output

```xml
<Document>
  <Header id="h1">
    <Title>Sample Document
      <Signature>
        <SignatureValue>
          eBM7WXmryCI17zjipYUVjkwBAxhK0P2Dx/SQEYUph+yq3oUREf3QTT6aE6g6zfBQHJ4s6Tx5tKqVk0gbQGbdVp6u6BQOCdsNhcbKeF2h4oar2KOC0rGgwn4YVQCB48Az2guoQHqqR/rAM2u2Qspcyn0eVokfM8GLeQK+HyAbpKufMkqjzW+mh3J3g+EOBlcRYe22q4EAwp0XWmi8kof9PYH2qaq0HxaBjgfiTgOKDzbfxsTiJOEMHCchFkzQ4GNbpacgjjAuQRnenJTuFD5d/eRV7TFAaH4og3cqZDgDBa52j/5WLDwLjI3xuhQBjzcyLBY5Q5oW35d+3Dnu0XR2+g==
        </SignatureValue>
      </Signature>
    </Title>
    <Author>John Doe</Author>
  </Header>
  <Body>
    <Section name="intro">
      <Paragraph>This is the first paragraph.</Paragraph>
      <Paragraph>This is the second paragraph.</Paragraph>
    </Section>
    <Section name="conclusion">
      <Paragraph>This is the conclusion.</Paragraph>
    </Section>
  </Body>
</Document>
```

## reference

- https://www.aleksey.com/xmlsec/
- https://github.com/lsh123/xmlsec
- https://github.com/lsh123/xmlsec/blob/master/examples/sign1-tmpl.xml
- https://github.com/lsh123/xmlsec/blob/master/examples/sign1-res.xml
- https://sgros.blogspot.com/2013/01/signing-xml-document-using-xmlsec1.html
- https://gitlab.com/reinis-mazeiks/xml_c14n
- https://docs.rs/serde-xml-rs/latest/serde_xml_rs/#attributes
- https://github.com/tafia/quick-xml
- 
