# Chemical Vector Graphics Specification

## Metadata

Specification version: This is an unversioned working document; the first versioned draft will be 0.1
Copyright: 2025 Matthew J. Milner
License: CC BY

## Introduction

- Chemical Vector Graphics (CVG) is a file format that functions as a portable, easily viewable image format while simultaneously storing full information on 2D chemical structure representation.
- CVG achieves this by being a strict subset of the SVG format.
- A "CVG" is simply an SVG file that complies with this CVG specification.
- By being a standards-compliant SVG image, a CVG can be viewed across many browsers, many systems, and many programs without requiring explicit additional support for CVG, and can be included or embedded in a vast number of places without issue, including within common document formats.
- The inclusion of the chemical semantics makes CVGs machine-readable.
	- While tools have been developed to infer chemical information from standard bitmap or vector images, these must make assumptions, whereas the information is explicit within a CVG, ensuring that the chemistry is interpreted in the way its author intended.
- SVG is flexible and there are many ways that a simple covalent bond could be drawn and positioned. CVG standardizes how chemical objects are described in SVG. Note that this is not the same as standardizing how chemical objects should be visually represented, which is the concern of the IUPAC Chemical Nomenclature and Structural Representation division.
- CVG is aimed at wide adoption and eventual standardization under the stewardship of IUPAC or some other international or national chemical organization.

## Basics

- A "CVG" is a representation of a chemical structure in XML code. A "CVG file" is a text file containing that code.
- A CVG file should use either the standard `.svg` suffix, or the `.cvg` suffix.
	- The former is recommended for general use as it enables the key feature of CVG: that it is widely recognized and displayed as a standard SVG.
	- This recommendation would change if the CVG MIME-type were to become well-supported and files with `.cvg` are treated as SVGs by most software.
	- At the present time, `.cvg` should only be used when it is of high importance to distinguish a CVG from non-CVG SVGs.
- A CVG is fully compliant with SVG 1.1.
- A CVG is restricted to the "SVG-static" feature subset.
	- Animation support would, if implemented in future, be an extension of the basic specification, due to the burden it would impose on processors of CVG to provide support.
	- Use of features from "SVG-dynamic", such as `<script>` elements, is explicitly forbidden for security reasons.
- Which SVG elements may be used in a CVG, and how, is generally very restrictive; see below.
- A CVG adhering to this specification and versions of this specification <= 2.0 will continue to be based on SVG 1.1 and should not use SVG >=2.0.
	- Whether version 2.0 or some other higher, future version of the CVG specification would be based on SVG 2.0 is undecided.
- A CVG must always be encoded as UTF-8 text.
- Spaces (0x20) should be used for whitespace, not tabs (0x09).
- Newlines should be LF (0x0A), not CRLF (0x0D 0x0A) or other variations.
	- Parsers should be tolerant and also parse CRLF as a newline character, but should normalize it to LF.
- The closing `</svg>` bracket of a CVG should be followed with a LF, even at the end of a file (a terminal newline).
	- This ensures maximum compatibility with broader software.
	- Parsers should tolerate a missing terminal newline, but should normalize output to include them.
- Chemical information is stored in a CVG using attributes in the `cvg:` namespace.
- Compliant CVG software implementations carrying out round-trip processing should aim to be content-preserving, and *must* be content-preserving for any `cvg` attributes.
- Comments are permitted in a CVG but preservation should not be expected.
- [Graphics elements](https://developer.mozilla.org/en-US/docs/Web/SVG/Element#graphics_elements) (`<circle>` `<ellipse>`, `<image>`, `<line>`, `<path>`, `<polygon>`, `<polyline>`, `<rect>`, `<text>`, `<use>`) should be contained on a single line.
	- As one of these elements generally represents a single chemical entity, this improves readability.
- Lengths should be specified in `px`.

## Glossary

- **element**
	- An SVG element.
- **entity**
	- An individual molecular entity such as an atom, bond, or molecule; see the [IUPAC Gold Book](https://goldbook.iupac.org/terms/view/M03986)
- **role**
	- A type of molecular entity or other object with meaning in CVG's chemical semantics

## Top-level structure

- A CVG consists of an SVG root element indicated as being a CVG through metadata. All contents of the CVG are entirely enclosed within the top-level `<svg>` element.
- The opening and closing brackets of a CVG should contain:
```xml
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" xmlns:cvg="http://www.github.com/matterhorn103/cvg" cvg:version="0.1">
...
</svg>
```
- Other attributes are permitted in the opening bracket.
- In particular, both `width` and `height` should be specified.
- Once a canonical location for this specification has been chosen, the CVG namespace attribute should link to that location.
- The value of the `cvg:version` attribute should reflect the version of this specification that is adhered to.
- An XML header such as `<?xml version="1.0" encoding="UTF-8" standalone="no"?>` should generally not be included in a CVG file, but parsers should tolerate its presence.
- As a single CVG is capable of representing an unlimited number of chemical structures within it, it should not generally be necessary to have multiple CVGs in a single file and a "CVG file" is expected to contain only a single CVG.
- Parsers should therefore generally ignore anything preceding the first occurrence of the `<svg` string and anything after the corresponding `</svg>` tag.

## IDs

- SVG ID strings should be formed by combining the abbreviated form of the `cvg:role` string with an integer.
- ID strings must be unique.
- Numbering is independent for each role, so `a1` and `b1` may coexist.
- Numbering should start at 1.
- Numbering should be done in an essentially ascending fashion. However, there is no obligation to ensure continuity of the IDs used and gaps in the sequence are permitted. Parsers must tolerate gaps, but round-trips may normalize IDs if desired. There is also no obligation to list entities in ascending ID order in a file.

## CVG attributes

- CVG namespace attributes should come before other normal SVG attributes
