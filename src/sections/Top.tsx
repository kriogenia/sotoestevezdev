import styled from "styled-components"
import LanguageSelector from "../components/LanguageSelector"
import ThemeSwitch from "../components/ThemeSwitch"

const Container = styled.div`
	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: end;
	padding: 10px;
	gap: 10px;
`

const Top = () => {
	return (
		<Container>
			<LanguageSelector />
          <ThemeSwitch />
		</Container>
	)
}

export default Top;