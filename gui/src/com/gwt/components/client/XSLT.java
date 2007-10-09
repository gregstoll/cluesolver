/**
 * Created on Aug 15, 2006
 */
package com.gwt.components.client.xslt;

import com.google.gwt.core.client.JavaScriptException;
import com.google.gwt.core.client.JavaScriptObject;

/**
 * Wraps the XSLT functions from Sarissa, http://sarissa.sourceforge.net/doc/
 * @author Eric Bessette <ebessette@gmail.com>
 */
public class XSLT {

	/**
	 * The JavaScript XSLT Processor
	 */
	private static final JavaScriptObject	xsltProcessor	= XSLT.initialize();

	/**
	 * @param xmlNode The input xml node
	 * @param xslNode The xsl stylesheet node
	 * @return The processed document, as XML text in a string
	 * @throws XSLTProcessException Thrown if the transformation process throws
	 *         an error
	 */
	public static String process( Node xmlNode, Node xslNode ) throws XSLTProcessException {

		if ( XSLT.xsltProcessor == null ) {
			return null;
		}

		try {
			return processImpl( xmlNode.getJSNode(), xslNode.getJSNode() );
		}
		catch ( JavaScriptException jse ) {
			throw new XSLTProcessException( jse );
		}
	}

	/**
	 * Transform an XML node using XSL
	 * @param xmlNode The input xml node
	 * @param xslNode The xsl stylesheet node
	 * @return The processed document, as XML text in a string
	 */
	private static native String processImpl( JavaScriptObject xmlNode, JavaScriptObject xslNode ) throws JavaScriptException/*-{
	 
	 @com.gwt.components.client.xslt.XSLT::xsltProcessor.importStylesheet( xslNode );

	 var domDoc = @com.gwt.components.client.xslt.XSLT::xsltProcessor.transformToDocument( xmlNode );
	 
	 if ( $wnd.Sarissa.getParseErrorText( domDoc ) != $wnd.Sarissa.PARSED_OK ) {
	 throw new Exception( $wnd.Sarissa.getParseErrorText( domDoc ) );
	 }
	 
	 return new $wnd.XMLSerializer().serializeToString(domDoc);
	 
	 }-*/;

	/**
	 * Initialize the XSLT processor
	 * @return A new XSLT processor
	 */
	private static native JavaScriptObject initialize() /*-{
	 return new $wnd.XSLTProcessor();
	 }-*/;
}
