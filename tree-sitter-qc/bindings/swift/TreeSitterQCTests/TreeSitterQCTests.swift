import XCTest
import SwiftTreeSitter
import TreeSitterQc

final class TreeSitterQcTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_qc())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading QC grammar")
    }
}
